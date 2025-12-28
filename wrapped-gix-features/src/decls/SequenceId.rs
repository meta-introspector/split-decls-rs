macro_rules! SequenceId {
    () => {
        # [doc = " A counter for items that are in sequence, to be able to put them back into original order later."] pub type SequenceId = usize ;
    };
}

SequenceId!();