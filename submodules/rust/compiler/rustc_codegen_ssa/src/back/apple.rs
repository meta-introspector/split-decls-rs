mkuse!{use std :: ffi :: OsString ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use std :: process :: Command ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: SymbolExportKind ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_target :: spec :: Target ;}
mkuse!{pub (super) use rustc_target :: spec :: apple :: OSVersion ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: errors :: { XcrunError , XcrunSdkPathWarning } ;}
mkuse!{use crate :: fluent_generated as fluent ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! sdk_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sdk_name in module {}", module_path!());
    };
}

mkfn!{
    sdk_name_introspect!();
    # [doc = " The canonical name of the desired SDK for a given target."] pub (super) fn sdk_name (target : & Target) -> & 'static str { match (& * target . os , & * target . env) { ("macos" , "") => "MacOSX" , ("ios" , "") => "iPhoneOS" , ("ios" , "sim") => "iPhoneSimulator" , ("ios" , "macabi") => "MacOSX" , ("tvos" , "") => "AppleTVOS" , ("tvos" , "sim") => "AppleTVSimulator" , ("visionos" , "") => "XROS" , ("visionos" , "sim") => "XRSimulator" , ("watchos" , "") => "WatchOS" , ("watchos" , "sim") => "WatchSimulator" , (os , abi) => unreachable ! ("invalid os '{os}' / abi '{abi}' combination for Apple target") , } }
}

macro_rules! macho_platform_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function macho_platform in module {}", module_path!());
    };
}

mkfn!{
    macho_platform_introspect!();
    pub (super) fn macho_platform (target : & Target) -> u32 { match (& * target . os , & * target . env) { ("macos" , _) => object :: macho :: PLATFORM_MACOS , ("ios" , "macabi") => object :: macho :: PLATFORM_MACCATALYST , ("ios" , "sim") => object :: macho :: PLATFORM_IOSSIMULATOR , ("ios" , _) => object :: macho :: PLATFORM_IOS , ("watchos" , "sim") => object :: macho :: PLATFORM_WATCHOSSIMULATOR , ("watchos" , _) => object :: macho :: PLATFORM_WATCHOS , ("tvos" , "sim") => object :: macho :: PLATFORM_TVOSSIMULATOR , ("tvos" , _) => object :: macho :: PLATFORM_TVOS , ("visionos" , "sim") => object :: macho :: PLATFORM_XROSSIMULATOR , ("visionos" , _) => object :: macho :: PLATFORM_XROS , _ => unreachable ! ("tried to get Mach-O platform for non-Apple target") , } }
}

macro_rules! add_data_and_relocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_data_and_relocation in module {}", module_path!());
    };
}

mkfn!{
    add_data_and_relocation_introspect!();
    # [doc = " Add relocation and section data needed for a symbol to be considered"] # [doc = " undefined by ld64."] # [doc = ""] # [doc = " The relocation must be valid, and hence must point to a valid piece of"] # [doc = " machine code, and hence this is unfortunately very architecture-specific."] # [doc = ""] # [doc = ""] # [doc = " # New architectures"] # [doc = ""] # [doc = " The values here are basically the same as emitted by the following program:"] # [doc = ""] # [doc = " ```c"] # [doc = " // clang -c foo.c -target $CLANG_TARGET"] # [doc = " void foo(void);"] # [doc = ""] # [doc = " extern int bar;"] # [doc = ""] # [doc = " void* foobar[2] = {"] # [doc = "     (void*)foo,"] # [doc = "     (void*)&bar,"] # [doc = "     // ..."] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Can be inspected with:"] # [doc = " ```console"] # [doc = " objdump --macho --reloc foo.o"] # [doc = " objdump --macho --full-contents foo.o"] # [doc = " ```"] pub (super) fn add_data_and_relocation (file : & mut object :: write :: Object < '_ > , section : object :: write :: SectionId , symbol : object :: write :: SymbolId , target : & Target , kind : SymbolExportKind ,) -> object :: write :: Result < () > { let authenticated_pointer = kind == SymbolExportKind :: Text && target . llvm_target . starts_with ("arm64e") ; let data : & [u8] = match target . pointer_width { _ if authenticated_pointer => & [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0x80] , 32 => & [0 ; 4] , 64 => & [0 ; 8] , pointer_width => unimplemented ! ("unsupported Apple pointer width {pointer_width:?}") , } ; if target . arch == "x86_64" { file . section_mut (section) . append_data (& [] , 16) ; } else { file . section_mut (section) . append_data (& [] , target . pointer_width as u64) ; } let offset = file . section_mut (section) . append_data (data , data . len () as u64) ; let flags = if authenticated_pointer { object :: write :: RelocationFlags :: MachO { r_type : object :: macho :: ARM64_RELOC_AUTHENTICATED_POINTER , r_pcrel : false , r_length : 3 , } } else if target . arch == "arm" { object :: write :: RelocationFlags :: MachO { r_type : object :: macho :: ARM_RELOC_VANILLA , r_pcrel : false , r_length : 2 , } } else { object :: write :: RelocationFlags :: Generic { kind : object :: RelocationKind :: Absolute , encoding : object :: RelocationEncoding :: Generic , size : target . pointer_width as u8 , } } ; file . add_relocation (section , object :: write :: Relocation { offset , addend : 0 , symbol , flags }) ? ; Ok (()) }
}

macro_rules! add_version_to_llvm_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_version_to_llvm_target in module {}", module_path!());
    };
}

mkfn!{
    add_version_to_llvm_target_introspect!();
    pub (super) fn add_version_to_llvm_target (llvm_target : & str , deployment_target : OSVersion ,) -> String { let mut components = llvm_target . split ("-") ; let arch = components . next () . expect ("apple target should have arch") ; let vendor = components . next () . expect ("apple target should have vendor") ; let os = components . next () . expect ("apple target should have os") ; let environment = components . next () ; assert_eq ! (components . next () , None , "too many LLVM triple components") ; assert ! (! os . contains (| c : char | c . is_ascii_digit ()) , "LLVM target must not already be versioned") ; let version = deployment_target . fmt_full () ; if let Some (env) = environment { format ! ("{arch}-{vendor}-{os}{version}-{env}") } else { format ! ("{arch}-{vendor}-{os}{version}") } }
}

macro_rules! get_sdk_root_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_sdk_root in module {}", module_path!());
    };
}

mkfn!{
    get_sdk_root_introspect!();
    pub (super) fn get_sdk_root (sess : & Session) -> Option < PathBuf > { let sdk_name = sdk_name (& sess . target) ; match xcrun_show_sdk_path (sdk_name , false) { Ok ((path , stderr)) => { if ! stderr . is_empty () { sess . dcx () . emit_warn (XcrunSdkPathWarning { sdk_name , stderr }) ; } Some (path) } Err (err) => { let mut diag = sess . dcx () . create_warn (err) ; diag . note (fluent :: codegen_ssa_xcrun_about) ; if let Some (developer_dir) = xcode_select_developer_dir () { diag . arg ("developer_dir" , & developer_dir) ; diag . note (fluent :: codegen_ssa_xcrun_found_developer_dir) ; if developer_dir . as_os_str () . to_string_lossy () . contains ("CommandLineTools") { if sdk_name != "MacOSX" { diag . help (fluent :: codegen_ssa_xcrun_command_line_tools_insufficient) ; } } } else { diag . help (fluent :: codegen_ssa_xcrun_no_developer_dir) ; } diag . emit () ; None } } }
}

macro_rules! xcrun_show_sdk_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xcrun_show_sdk_path in module {}", module_path!());
    };
}

mkfn!{
    xcrun_show_sdk_path_introspect!();
    # [doc = " Invoke `xcrun --sdk $sdk_name --show-sdk-path` to get the SDK path."] # [doc = ""] # [doc = " The exact logic that `xcrun` uses is unspecified (see `man xcrun` for a few details), and may"] # [doc = " change between macOS and Xcode versions, but it roughly boils down to finding the active"] # [doc = " developer directory, and then invoking `xcodebuild -sdk $sdk_name -version` to get the SDK"] # [doc = " details."] # [doc = ""] # [doc = " Finding the developer directory is roughly done by looking at, in order:"] # [doc = " - The `DEVELOPER_DIR` environment variable."] # [doc = " - The `/var/db/xcode_select_link` symlink (set by `xcode-select --switch`)."] # [doc = " - `/Applications/Xcode.app` (hardcoded fallback path)."] # [doc = " - `/Library/Developer/CommandLineTools` (hardcoded fallback path)."] # [doc = ""] # [doc = " Note that `xcrun` caches its result, but with a cold cache this whole operation can be quite"] # [doc = " slow, especially so the first time it's run after a reboot."] fn xcrun_show_sdk_path (sdk_name : & 'static str , verbose : bool ,) -> Result < (PathBuf , String) , XcrunError > { let mut cmd = Command :: new ("xcrun") ; if verbose { cmd . arg ("--verbose") ; } cmd . arg ("--sdk") ; cmd . arg (& sdk_name . to_lowercase ()) ; cmd . arg ("--show-sdk-path") ; let output = cmd . output () . map_err (| error | XcrunError :: FailedInvoking { sdk_name , command_formatted : format ! ("{cmd:?}") , error , }) ? ; let stderr = String :: from_utf8_lossy_owned (output . stderr) ; if ! stderr . is_empty () { debug ! (stderr , "original xcrun stderr") ; } let stderr = stderr . lines () . filter (| line | { ! line . contains ("Writing error result bundle") && ! line . contains ("Requested but did not find extension point with identifier") }) . join ("\n") ; if output . status . success () { Ok ((stdout_to_path (output . stdout) , stderr)) } else { let stdout = String :: from_utf8_lossy_owned (output . stdout) . trim () . to_string () ; Err (XcrunError :: Unsuccessful { sdk_name , command_formatted : format ! ("{cmd:?}") , stdout , stderr , }) } }
}

macro_rules! xcode_select_developer_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xcode_select_developer_dir in module {}", module_path!());
    };
}

mkfn!{
    xcode_select_developer_dir_introspect!();
    # [doc = " Invoke `xcode-select --print-path`, and return the current developer directory."] # [doc = ""] # [doc = " NOTE: We don't do any error handling here, this is only used as a canary in diagnostics (`xcrun`"] # [doc = " will have already emitted the relevant error information)."] fn xcode_select_developer_dir () -> Option < PathBuf > { let mut cmd = Command :: new ("xcode-select") ; cmd . arg ("--print-path") ; let output = cmd . output () . ok () ? ; if ! output . status . success () { return None ; } Some (stdout_to_path (output . stdout)) }
}

macro_rules! stdout_to_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stdout_to_path in module {}", module_path!());
    };
}

mkfn!{
    stdout_to_path_introspect!();
    fn stdout_to_path (mut stdout : Vec < u8 >) -> PathBuf { if let Some (b'\n') = stdout . last () { let _ = stdout . pop () . unwrap () ; } # [cfg (unix)] let path = < OsString as std :: os :: unix :: ffi :: OsStringExt > :: from_vec (stdout) ; # [cfg (not (unix))] let path = OsString :: from (String :: from_utf8 (stdout) . expect ("stdout must be UTF-8")) ; PathBuf :: from (path) }
}