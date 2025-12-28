macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! Utf8PathBuf {
    () => {
        deps!();
        # [doc = " An owned, mutable UTF-8 path (akin to [`String`])."] # [doc = ""] # [doc = " This type provides methods like [`push`] and [`set_extension`] that mutate"] # [doc = " the path in place. It also implements [`Deref`] to [`Utf8Path`], meaning that"] # [doc = " all methods on [`Utf8Path`] slices are available on [`Utf8PathBuf`] values as well."] # [doc = ""] # [doc = " [`push`]: Utf8PathBuf::push"] # [doc = " [`set_extension`]: Utf8PathBuf::set_extension"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " You can use [`push`] to build up a [`Utf8PathBuf`] from"] # [doc = " components:"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8PathBuf;"] # [doc = ""] # [doc = " let mut path = Utf8PathBuf::new();"] # [doc = ""] # [doc = " path.push(r\"C:\\\");"] # [doc = " path.push(\"windows\");"] # [doc = " path.push(\"system32\");"] # [doc = ""] # [doc = " path.set_extension(\"dll\");"] # [doc = " ```"] # [doc = ""] # [doc = " However, [`push`] is best used for dynamic situations. This is a better way"] # [doc = " to do this when you know all of the components ahead of time:"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8PathBuf;"] # [doc = ""] # [doc = " let path: Utf8PathBuf = [r\"C:\\\", \"windows\", \"system32.dll\"].iter().collect();"] # [doc = " ```"] # [doc = ""] # [doc = " We can still do better than this! Since these are all strings, we can use"] # [doc = " [`From::from`]:"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8PathBuf;"] # [doc = ""] # [doc = " let path = Utf8PathBuf::from(r\"C:\\windows\\system32.dll\");"] # [doc = " ```"] # [doc = ""] # [doc = " Which method works best depends on what kind of situation you're in."] # [derive (Clone , Default)] # [repr (transparent)] pub struct Utf8PathBuf (PathBuf) ;
    };
}

Utf8PathBuf!();