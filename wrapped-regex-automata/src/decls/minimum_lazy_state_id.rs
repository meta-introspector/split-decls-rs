macro_rules! deps {
    () => {
        ByteClasses!();
        LazyStateIDError!();
        LazyStateID!();
        DFA!();
    };
}

macro_rules! minimum_lazy_state_id {
    () => {
        deps!();
        # [doc = " Based on the minimum number of states required for a useful lazy DFA cache,"] # [doc = " this returns the minimum lazy state ID that must be representable."] # [doc = ""] # [doc = " It's not likely for this to have any impact 32-bit systems (or higher), but"] # [doc = " on 16-bit systems, the lazy state ID space is quite constrained and thus"] # [doc = " may be insufficient if our MIN_STATES value is (for some reason) too high."] fn minimum_lazy_state_id (classes : & ByteClasses ,) -> Result < LazyStateID , LazyStateIDError > { let stride = 1 << classes . stride2 () ; let min_state_index = MIN_STATES . checked_sub (1) . unwrap () ; LazyStateID :: new (min_state_index * stride) }
    };
}

minimum_lazy_state_id!()