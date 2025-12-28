macro_rules! deps {
    () => {
        Error!();
        ErrorExtensionValues!();
    };
}

macro_rules! ErrorExtensions {
    () => {
        deps!();
        # [doc = " An error which can be extended into a `Error`."] pub trait ErrorExtensions : Sized { # [doc = " Convert the error to a `Error`."] fn extend (& self) -> Error ; # [doc = " Add extensions to the error, using a callback to make the extensions."] fn extend_with < C > (self , cb : C) -> Error where C : FnOnce (& Self , & mut ErrorExtensionValues) , { let mut new_extensions = Default :: default () ; cb (& self , & mut new_extensions) ; let Error { message , source , extensions , } = self . extend () ; let mut extensions = extensions . unwrap_or_default () ; extensions . 0 . extend (new_extensions . 0) ; Error { message , source , extensions : Some (extensions) , } } }
    };
}

ErrorExtensions!()