macro_rules! OperatorClass {
    () => {
        # [derive (Debug)] pub enum OperatorClass { Range (Struct) , Await (Function) , Prefix (Function) , Index (Function) , Try (Function) , Bin (Function) , }
    };
}

OperatorClass!()