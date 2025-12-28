macro_rules! deps {
    () => {
        BitcodeSection!();
    };
}

macro_rules! EmitObj {
    () => {
        deps!();
        # [doc = " What kind of object file to emit."] # [derive (Clone , Copy , PartialEq)] pub enum EmitObj { None , Bitcode , ObjectCode (BitcodeSection) , }
    };
}

EmitObj!()