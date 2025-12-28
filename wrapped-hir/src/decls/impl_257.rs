macro_rules! deps {
    () => {
        Field!();
        TypeNs!();
        InstantiatedStruct!();
        InstantiatedField!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'db > InstantiatedStruct < 'db > { pub fn fields (self , db : & dyn HirDatabase) -> Vec < InstantiatedField < 'db > > { self . inner . id . fields (db) . fields () . iter () . map (| (id , _) | InstantiatedField { inner : Field { parent : self . inner . into () , id } , args : self . args , }) . collect () } pub fn ty (self , db : & 'db dyn HirDatabase) -> TypeNs < 'db > { let krate = self . inner . krate (db) ; let interner = DbInterner :: new_with (db , Some (krate . base ()) , None) ; let ty = db . ty (self . inner . id . into ()) ; TypeNs :: new (db , self . inner . id , ty . instantiate (interner , self . args)) } }
    };
}

impl_257!()