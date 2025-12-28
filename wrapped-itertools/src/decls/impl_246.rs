macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < I > Clone for Format < '_ , I > where I : Clone , { fn clone (& self) -> Self { struct PutBackOnDrop < 'r , 'a , I > { into : & 'r Format < 'a , I > , inner : Option < I > , } impl < I > Drop for PutBackOnDrop < '_ , '_ , I > { fn drop (& mut self) { self . into . inner . set (self . inner . take ()) } } let pbod = PutBackOnDrop { inner : self . inner . take () , into : self , } ; Self { inner : Cell :: new (pbod . inner . clone ()) , sep : self . sep , } } }
    };
}

impl_246!();