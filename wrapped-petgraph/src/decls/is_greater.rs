macro_rules! is_greater {
    () => {
        # [doc = " Helper to check if the distance map is greater then a specific value"] fn is_greater < K : PartialOrd > (m_dist : & mut Option < Vec < Vec < K > > > , i : usize , j : usize , value : K ,) -> bool { if let Some (dist) = m_dist { return dist [i] [j] > value ; } false }
    };
}

is_greater!();