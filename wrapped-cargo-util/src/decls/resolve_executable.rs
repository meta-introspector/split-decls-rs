macro_rules! resolve_executable {
    () => {
        # [doc = " Returns the absolute path of where the given executable is located based"] # [doc = " on searching the `PATH` environment variable."] # [doc = ""] # [doc = " Returns an error if it cannot be found."] pub fn resolve_executable (exec : & Path) -> Result < PathBuf > { if exec . components () . count () == 1 { let paths = env :: var_os ("PATH") . ok_or_else (| | anyhow :: format_err ! ("no PATH")) ? ; let candidates = env :: split_paths (& paths) . flat_map (| path | { let candidate = path . join (& exec) ; let with_exe = if env :: consts :: EXE_EXTENSION . is_empty () { None } else { Some (candidate . with_extension (env :: consts :: EXE_EXTENSION)) } ; iter :: once (candidate) . chain (with_exe) }) ; for candidate in candidates { if candidate . is_file () { return Ok (candidate) ; } } anyhow :: bail ! ("no executable for `{}` found in PATH" , exec . display ()) } else { Ok (exec . into ()) } }
    };
}

resolve_executable!()