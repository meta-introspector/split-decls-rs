macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PrefixComponent!();
        Utf8Components!();
    };
}

macro_rules! Utf8Component {
    () => {
        deps!();
        # [doc = " A single component of a path."] # [doc = ""] # [doc = " A [`Utf8Component`] roughly corresponds to a substring between path separators"] # [doc = " (`/` or `\\`)."] # [doc = ""] # [doc = " This `enum` is created by iterating over [`Utf8Components`], which in turn is"] # [doc = " created by the [`components`](Utf8Path::components) method on [`Utf8Path`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use camino::{Utf8Component, Utf8Path};"] # [doc = ""] # [doc = " let path = Utf8Path::new(\"/tmp/foo/bar.txt\");"] # [doc = " let components = path.components().collect::<Vec<_>>();"] # [doc = " assert_eq!(&components, &["] # [doc = "     Utf8Component::RootDir,"] # [doc = "     Utf8Component::Normal(\"tmp\"),"] # [doc = "     Utf8Component::Normal(\"foo\"),"] # [doc = "     Utf8Component::Normal(\"bar.txt\"),"] # [doc = " ]);"] # [doc = " ```"] # [derive (Copy , Clone , Eq , PartialEq , Hash , Ord , PartialOrd)] pub enum Utf8Component < 'a > { # [doc = " A Windows path prefix, e.g., `C:` or `\\\\server\\share`."] # [doc = ""] # [doc = " There is a large variety of prefix types, see [`Utf8Prefix`]'s documentation"] # [doc = " for more."] # [doc = ""] # [doc = " Does not occur on Unix."] Prefix (Utf8PrefixComponent < 'a >) , # [doc = " The root directory component, appears after any prefix and before anything else."] # [doc = ""] # [doc = " It represents a separator that designates that a path starts from root."] RootDir , # [doc = " A reference to the current directory, i.e., `.`."] CurDir , # [doc = " A reference to the parent directory, i.e., `..`."] ParentDir , # [doc = " A normal component, e.g., `a` and `b` in `a/b`."] # [doc = ""] # [doc = " This variant is the most common one, it represents references to files"] # [doc = " or directories."] Normal (& 'a str) , }
    };
}

Utf8Component!()