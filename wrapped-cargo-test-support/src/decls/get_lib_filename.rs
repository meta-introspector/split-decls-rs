macro_rules! get_lib_filename {
    () => {
        # [doc = " Get the filename for a library."] # [doc = ""] # [doc = " `kind` should be one of:"] # [doc = " - `lib`"] # [doc = " - `rlib`"] # [doc = " - `staticlib`"] # [doc = " - `dylib`"] # [doc = " - `proc-macro`"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use cargo_test_support::paths::get_lib_filename;"] # [doc = " get_lib_filename(\"foo\", \"dylib\");"] # [doc = " ```"] # [doc = " would return:"] # [doc = " - macOS: `\"libfoo.dylib\"`"] # [doc = " - Windows: `\"foo.dll\"`"] # [doc = " - Unix: `\"libfoo.so\"`"] pub fn get_lib_filename (name : & str , kind : & str) -> String { let prefix = get_lib_prefix (kind) ; let extension = get_lib_extension (kind) ; format ! ("{}{}.{}" , prefix , name , extension) }
    };
}

get_lib_filename!();