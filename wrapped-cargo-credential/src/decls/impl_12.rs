macro_rules! deps {
    () => {
        Secret!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T > Secret < T > { # [doc = " Unwraps the contained value."] # [doc = ""] # [doc = " Use of this method marks the boundary of where the contained value is"] # [doc = " hidden."] pub fn expose (self) -> T { self . inner } # [doc = " Converts a `Secret<T>` to a `Secret<&T::Target>`."] # [doc = " ```"] # [doc = " # use cargo_credential::Secret;"] # [doc = " let owned: Secret<String> = Secret::from(String::from(\"token\"));"] # [doc = " let borrowed: Secret<&str> = owned.as_deref();"] # [doc = " ```"] pub fn as_deref (& self) -> Secret < & < T as Deref > :: Target > where T : Deref , { Secret :: from (self . inner . deref ()) } # [doc = " Converts a `Secret<T>` to a `Secret<&T>`."] pub fn as_ref (& self) -> Secret < & T > { Secret :: from (& self . inner) } # [doc = " Converts a `Secret<T>` to a `Secret<U>` by applying `f` to the contained value."] pub fn map < U , F > (self , f : F) -> Secret < U > where F : FnOnce (T) -> U , { Secret :: from (f (self . inner)) } }
    };
}

impl_12!()