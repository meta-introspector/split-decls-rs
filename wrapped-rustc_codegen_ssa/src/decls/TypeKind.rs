macro_rules! TypeKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] pub enum TypeKind { Void , Half , Float , Double , X86_FP80 , FP128 , PPC_FP128 , Label , Integer , Function , Struct , Array , Pointer , Vector , Metadata , Token , ScalableVector , BFloat , X86_AMX , }
    };
}

TypeKind!()