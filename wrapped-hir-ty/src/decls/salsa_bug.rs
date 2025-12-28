macro_rules! deps {
    () => {
        TestDB!();
    };
}

macro_rules! salsa_bug {
    () => {
        deps!();
        # [test] fn salsa_bug () { let (mut db , pos) = TestDB :: with_position ("
        //- /lib.rs
        trait Index {
            type Output;
        }

        type Key<S: UnificationStoreBase> = <S as UnificationStoreBase>::Key;

        pub trait UnificationStoreBase: Index<Output = Key<Self>> {
            type Key;

            fn len(&self) -> usize;
        }

        pub trait UnificationStoreMut: UnificationStoreBase {
            fn push(&mut self, value: Self::Key);
        }

        fn main() {
            let x = 1;
            x.push(1);$0
        }
    " ,) ; crate :: attach_db (& db , | | { let module = db . module_for_file (pos . file_id . file_id (& db)) ; let crate_def_map = module . def_map (& db) ; visit_module (& db , crate_def_map , module . local_id , & mut | def | { db . infer (match def { ModuleDefId :: FunctionId (it) => it . into () , ModuleDefId :: EnumVariantId (it) => it . into () , ModuleDefId :: ConstId (it) => it . into () , ModuleDefId :: StaticId (it) => it . into () , _ => return , }) ; }) ; }) ; let new_text = "
        //- /lib.rs
        trait Index {
            type Output;
        }

        type Key<S: UnificationStoreBase> = <S as UnificationStoreBase>::Key;

        pub trait UnificationStoreBase: Index<Output = Key<Self>> {
            type Key;

            fn len(&self) -> usize;
        }

        pub trait UnificationStoreMut: UnificationStoreBase {
            fn push(&mut self, value: Self::Key);
        }

        fn main() {

            let x = 1;
            x.push(1);
        }
    " ; db . set_file_text (pos . file_id . file_id (& db) , new_text) ; crate :: attach_db (& db , | | { let module = db . module_for_file (pos . file_id . file_id (& db)) ; let crate_def_map = module . def_map (& db) ; visit_module (& db , crate_def_map , module . local_id , & mut | def | { db . infer (match def { ModuleDefId :: FunctionId (it) => it . into () , ModuleDefId :: EnumVariantId (it) => it . into () , ModuleDefId :: ConstId (it) => it . into () , ModuleDefId :: StaticId (it) => it . into () , _ => return , }) ; }) ; }) }
    };
}

salsa_bug!();