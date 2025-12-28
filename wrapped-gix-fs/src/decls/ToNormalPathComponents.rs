macro_rules! ToNormalPathComponents {
    () => {
        # [doc = " Obtain an iterator over `OsStr`-components which are normal, none-relative and not absolute."] pub trait ToNormalPathComponents { # [doc = " Return an iterator over the normal components of a path, without the separator."] fn to_normal_path_components (& self) -> impl Iterator < Item = Result < & OsStr , to_normal_path_components :: Error > > ; }
    };
}

ToNormalPathComponents!();