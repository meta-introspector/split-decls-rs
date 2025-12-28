macro_rules! deps {
    () => {
        TypeNs!();
        InstantiatedEnum!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'db > InstantiatedEnum < 'db > { pub fn ty (self , db : & 'db dyn HirDatabase) -> TypeNs < 'db > { let krate = self . inner . krate (db) ; let interner = DbInterner :: new_with (db , Some (krate . base ()) , None) ; let ty = db . ty (self . inner . id . into ()) ; TypeNs :: new (db , self . inner . id , ty . instantiate (interner , self . args)) } }
    };
}

impl_49!()