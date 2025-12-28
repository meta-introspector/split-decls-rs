macro_rules! deps {
    () => {
        GenericSequence!();
    };
}

macro_rules! SequenceItem {
    () => {
        deps!();
        # [doc = " Accessor for `GenericSequence` item type, which is really `IntoIterator::Item`"] # [doc = ""] # [doc = " For deeply nested generic mapped sequence types, like shown in `tests/generics.rs`,"] # [doc = " this can be useful for keeping things organized."] pub type SequenceItem < T > = < T as IntoIterator > :: Item ;
    };
}

SequenceItem!()