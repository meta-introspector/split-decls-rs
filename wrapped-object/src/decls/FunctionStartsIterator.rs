macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! FunctionStartsIterator {
    () => {
        deps!();
        # [doc = " Iterator over the function starts in a `LC_FUNCTION_STARTS` load command."] # [derive (Debug , Default , Clone , Copy)] pub struct FunctionStartsIterator < 'data > { data : Bytes < 'data > , addr : u64 , }
    };
}

FunctionStartsIterator!()