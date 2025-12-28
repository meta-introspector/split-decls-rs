macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! Utf8Path {
    () => {
        deps!();
        # [doc = " A slice of a UTF-8 path (akin to [`str`])."] # [doc = ""] # [doc = " This type supports a number of operations for inspecting a path, including"] # [doc = " breaking the path into its components (separated by `/` on Unix and by either"] # [doc = " `/` or `\\` on Windows), extracting the file name, determining whether the path"] # [doc = " is absolute, and so on."] # [doc = ""] # [doc = " This is an *unsized* type, meaning that it must always be used behind a"] # [doc = " pointer like `&` or [`Box`]. For an owned version of this type,"] # [doc = " see [`Utf8PathBuf`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8Path;"] # [doc = ""] # [doc = " // Note: this example does work on Windows"] # [doc = " let path = Utf8Path::new(\"./foo/bar.txt\");"] # [doc = ""] # [doc = " let parent = path.parent();"] # [doc = " assert_eq!(parent, Some(Utf8Path::new(\"./foo\")));"] # [doc = ""] # [doc = " let file_stem = path.file_stem();"] # [doc = " assert_eq!(file_stem, Some(\"bar\"));"] # [doc = ""] # [doc = " let extension = path.extension();"] # [doc = " assert_eq!(extension, Some(\"txt\"));"] # [doc = " ```"] # [repr (transparent)] pub struct Utf8Path (Path) ;
    };
}

Utf8Path!();