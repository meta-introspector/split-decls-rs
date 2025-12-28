macro_rules! var {
    () => {
        # [doc = " Returns the contents of an environment variable of `name` with some special handling for"] # [doc = " certain environment variables (like `HOME`) for platform compatibility."] pub fn var (name : & str) -> Option < OsString > { if name == "HOME" { home_dir () . map (PathBuf :: into_os_string) } else { std :: env :: var_os (name) } }
    };
}

var!()