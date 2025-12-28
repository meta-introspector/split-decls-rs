macro_rules! deps {
    () => {
        Mode!();
        Store!();
        Handle!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < S > Clone for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn clone (& self) -> Self { super :: Handle { store : self . store . clone () , refresh : self . refresh , ignore_replacements : self . ignore_replacements , token : { let token = self . store . register_handle () ; match self . token . as_ref () . expect ("token is always set here ") { handle :: Mode :: DeletedPacksAreInaccessible => token , handle :: Mode :: KeepDeletedPacksAvailable => self . store . upgrade_handle (token) , } . into () } , inflate : RefCell :: new (Default :: default ()) , snapshot : RefCell :: new (self . store . collect_snapshot ()) , max_recursion_depth : self . max_recursion_depth , packed_object_count : Default :: default () , } } }
    };
}

impl_71!();