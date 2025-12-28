macro_rules! impl_969 {
    () => {
        impl < Si , Item , U , Fut , F > Clone for With < Si , Item , U , Fut , F > where Si : Clone , F : Clone , Fut : Clone , { fn clone (& self) -> Self { Self { state : self . state . clone () , sink : self . sink . clone () , f : self . f . clone () , _phantom : PhantomData , } } }
    };
}

impl_969!();