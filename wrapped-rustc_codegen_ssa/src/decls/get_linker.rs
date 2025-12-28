macro_rules! deps {
    () => {
        WasmLd!();
        BpfLinker!();
        LlbcLinker!();
        EmLinker!();
        AixLinker!();
        MsvcLinker!();
        L4Bender!();
        Command!();
        GccLinker!();
        PtxLinker!();
        Linker!();
    };
}

macro_rules! get_linker {
    () => {
        deps!();
        # [doc = " The third parameter is for env vars, used on windows to set up the"] # [doc = " path for MSVC to find its DLLs, and gcc to find its bundled"] # [doc = " toolchain"] pub (crate) fn get_linker < 'a > (sess : & 'a Session , linker : & Path , flavor : LinkerFlavor , self_contained : bool , target_cpu : & 'a str ,) -> Box < dyn Linker + 'a > { let msvc_tool = windows_registry :: find_tool (& sess . target . arch , "link.exe") ; let mut cmd = match linker . to_str () { Some (linker) if cfg ! (windows) && linker . ends_with (".bat") => Command :: bat_script (linker) , _ => match flavor { LinkerFlavor :: Gnu (Cc :: No , Lld :: Yes) | LinkerFlavor :: Darwin (Cc :: No , Lld :: Yes) | LinkerFlavor :: WasmLld (Cc :: No) | LinkerFlavor :: Msvc (Lld :: Yes) => Command :: lld (linker , flavor . lld_flavor ()) , LinkerFlavor :: Msvc (Lld :: No) if sess . opts . cg . linker . is_none () && sess . target . linker . is_none () => { Command :: new (msvc_tool . as_ref () . map_or (linker , | t | t . path ())) } _ => Command :: new (linker) , } , } ; let t = & sess . target ; if matches ! (flavor , LinkerFlavor :: Msvc (..)) && t . vendor == "uwp" { if let Some (ref tool) = msvc_tool { let original_path = tool . path () ; if let Some (root_lib_path) = original_path . ancestors () . nth (4) { let arch = match t . arch . as_ref () { "x86_64" => Some ("x64") , "x86" => Some ("x86") , "aarch64" => Some ("arm64") , "arm" => Some ("arm") , _ => None , } ; if let Some (ref a) = arch { let mut arg = OsString :: from ("/LIBPATH:") ; arg . push (format ! ("{}\\lib\\{}\\store" , root_lib_path . display () , a)) ; cmd . arg (& arg) ; } else { warn ! ("arch is not supported") ; } } else { warn ! ("MSVC root path lib location not found") ; } } else { warn ! ("link.exe not found") ; } } let mut new_path = sess . get_tools_search_paths (self_contained) ; let mut msvc_changed_path = false ; if sess . target . is_like_msvc && let Some (ref tool) = msvc_tool { cmd . args (tool . args ()) ; for (k , v) in tool . env () { if k == "PATH" { new_path . extend (env :: split_paths (v)) ; msvc_changed_path = true ; } else { cmd . env (k , v) ; } } } if ! msvc_changed_path && let Some (path) = env :: var_os ("PATH") { new_path . extend (env :: split_paths (& path)) ; } cmd . env ("PATH" , env :: join_paths (new_path) . unwrap ()) ; assert ! (cmd . get_args () . is_empty () || sess . target . vendor == "uwp") ; match flavor { LinkerFlavor :: Unix (Cc :: No) if sess . target . os == "l4re" => { Box :: new (L4Bender :: new (cmd , sess)) as Box < dyn Linker > } LinkerFlavor :: Unix (Cc :: No) if sess . target . os == "aix" => { Box :: new (AixLinker :: new (cmd , sess)) as Box < dyn Linker > } LinkerFlavor :: WasmLld (Cc :: No) => Box :: new (WasmLd :: new (cmd , sess)) as Box < dyn Linker > , LinkerFlavor :: Gnu (cc , _) | LinkerFlavor :: Darwin (cc , _) | LinkerFlavor :: WasmLld (cc) | LinkerFlavor :: Unix (cc) => Box :: new (GccLinker { cmd , sess , target_cpu , hinted_static : None , is_ld : cc == Cc :: No , is_gnu : flavor . is_gnu () , uses_lld : flavor . uses_lld () , }) as Box < dyn Linker > , LinkerFlavor :: Msvc (..) => Box :: new (MsvcLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: EmCc => Box :: new (EmLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Bpf => Box :: new (BpfLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Llbc => Box :: new (LlbcLinker { cmd , sess }) as Box < dyn Linker > , LinkerFlavor :: Ptx => Box :: new (PtxLinker { cmd , sess }) as Box < dyn Linker > , } }
    };
}

get_linker!()