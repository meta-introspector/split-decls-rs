macro_rules! impl_154 {
    () => {
        impl PackageName { # [doc = " Coerce a value to be a validate package name"] # [doc = ""] # [doc = " Replaces invalid values with `placeholder`"] pub fn sanitize (name : impl AsRef < str > , placeholder : char) -> Self { PackageName (restricted_names :: sanitize_package_name (name . as_ref () , placeholder ,)) } }
    };
}

impl_154!()