macro_rules! deps {
    () => {
        Reader!();
        IndexSectionId!();
        UnitIndex!();
    };
}

macro_rules! UnitIndexSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator over the section offsets and sizes for a row in a `UnitIndex`."] # [derive (Debug , Clone)] pub struct UnitIndexSectionIterator < 'index , R : Reader > { sections : slice :: Iter < 'index , IndexSectionId > , offsets : R , sizes : R , }
    };
}

UnitIndexSectionIterator!()