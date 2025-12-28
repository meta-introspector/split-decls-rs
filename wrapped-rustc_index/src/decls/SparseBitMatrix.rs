macro_rules! deps {
    () => {
        Idx!();
        DenseBitSet!();
        IndexVec!();
    };
}

macro_rules! SparseBitMatrix {
    () => {
        deps!();
        # [doc = " A fixed-column-size, variable-row-size 2D bit matrix with a moderately"] # [doc = " sparse representation."] # [doc = ""] # [doc = " Initially, every row has no explicit representation. If any bit within a row"] # [doc = " is set, the entire row is instantiated as `Some(<DenseBitSet>)`."] # [doc = " Furthermore, any previously uninstantiated rows prior to it will be"] # [doc = " instantiated as `None`. Those prior rows may themselves become fully"] # [doc = " instantiated later on if any of their bits are set."] # [doc = ""] # [doc = " `R` and `C` are index types used to identify rows and columns respectively;"] # [doc = " typically newtyped `usize` wrappers, but they can also just be `usize`."] # [derive (Clone , Debug)] pub struct SparseBitMatrix < R , C > where R : Idx , C : Idx , { num_columns : usize , rows : IndexVec < R , Option < DenseBitSet < C > > > , }
    };
}

SparseBitMatrix!();