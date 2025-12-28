macro_rules! fan {
    () => {
        # [doc = " Returns the `index` at which the following `index + 1` value is not an increment over the value at `index`."] pub fn fan (data : & [u32]) -> Option < usize > { data . windows (2) . enumerate () . find_map (| (win_index , v) | (v [0] > v [1]) . then_some (win_index)) }
    };
}

fan!();