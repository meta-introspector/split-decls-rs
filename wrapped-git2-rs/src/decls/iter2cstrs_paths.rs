macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! iter2cstrs_paths {
    () => {
        deps!();
        # [doc = " Converts an iterator of repo paths into a git2-compatible array of cstrings."] # [doc = ""] # [doc = " Only use this for repo-relative paths or pathspecs."] # [doc = ""] # [doc = " See `iter2cstrs` for more details."] pub fn iter2cstrs_paths < T , I > (iter : I ,) -> Result < (Vec < CString > , Vec < * const c_char > , raw :: git_strarray) , Error > where T : IntoCString , I : IntoIterator < Item = T > , { let cstrs = iter . into_iter () . map (| i | fixup_windows_path (i . into_c_string () ?)) . collect :: < Result < Vec < CString > , _ > > () ? ; iter2cstrs (cstrs) }
    };
}

iter2cstrs_paths!();