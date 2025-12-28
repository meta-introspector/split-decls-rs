macro_rules! is_iterator_singleton {
    () => {
        # [doc = " Returns `Some(iterator.next())` if it has exactly one item, and `None` otherwise."] fn is_iterator_singleton < T > (mut iterator : impl Iterator < Item = T >) -> Option < T > { match (iterator . next () , iterator . next ()) { (_ , Some (_)) => None , (first , _) => first , } }
    };
}

is_iterator_singleton!()