macro_rules! ParentIteratorState {
    () => {
        # [derive (Debug)] enum ParentIteratorState < 'a > { First , Second , Extra (Chunks < 'a , u8 >) , Exhausted , }
    };
}

ParentIteratorState!();