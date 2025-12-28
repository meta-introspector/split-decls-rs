macro_rules! deps {
    () => {
        SelfContainedLinkerMissing!();
        Linker!();
    };
}

macro_rules! add_lld_args {
    () => {
        deps!();
        # [doc = " When using the linker flavors opting in to `lld`, add the necessary paths and arguments to"] # [doc = " invoke it:"] # [doc = " - when the self-contained linker flag is active: the build of `lld` distributed with rustc,"] # [doc = " - or any `lld` available to `cc`."] fn add_lld_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , self_contained_components : LinkSelfContainedComponents ,) { debug ! ("add_lld_args requested, flavor: '{:?}', target self-contained components: {:?}" , flavor , self_contained_components ,) ; if ! (flavor . uses_cc () && flavor . uses_lld ()) { return ; } let self_contained_cli = sess . opts . cg . link_self_contained . is_linker_enabled () ; let self_contained_target = self_contained_components . is_linker_enabled () ; let self_contained_linker = self_contained_cli || self_contained_target ; if self_contained_linker && ! sess . opts . cg . link_self_contained . is_linker_disabled () { let mut linker_path_exists = false ; for path in sess . get_tools_search_paths (false) { let linker_path = path . join ("gcc-ld") ; linker_path_exists |= linker_path . exists () ; cmd . cc_arg ({ let mut arg = OsString :: from ("-B") ; arg . push (linker_path) ; arg }) ; } if ! linker_path_exists { sess . dcx () . emit_fatal (errors :: SelfContainedLinkerMissing) ; } } if ! sess . target . is_like_wasm { cmd . cc_arg ("-fuse-ld=lld") ; } if ! flavor . is_gnu () { if sess . target . linker_flavor != sess . host . linker_flavor { cmd . cc_arg (format ! ("--target={}" , versioned_llvm_target (sess))) ; } } }
    };
}

add_lld_args!();