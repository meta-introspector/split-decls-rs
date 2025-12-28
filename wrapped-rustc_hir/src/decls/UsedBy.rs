macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! UsedBy {
    () => {
        deps!();
        # [doc = " There are three valid forms of the attribute:"] # [doc = " `#[used]`, which is semantically equivalent to `#[used(linker)]` except that the latter is currently unstable."] # [doc = " `#[used(compiler)]`"] # [doc = " `#[used(linker)]`"] # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum UsedBy { Compiler , Linker , }
    };
}

UsedBy!()