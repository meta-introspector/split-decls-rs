macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        DoubleEndedFallibleStreamingIterator!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use super :: * ; fn _is_object_safe (_ : & FallibleStreamingIterator < Item = () , Error = () >) { } fn _is_object_safe_double (_ : & DoubleEndedFallibleStreamingIterator < Item = () , Error = () >) { } }
    };
}

test!()