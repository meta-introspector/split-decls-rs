macro_rules! set_object {
    () => {
        # [doc = " Helper function to copy a value to a 2D array"] fn set_object < K : Clone > (m_dist : & mut Option < Vec < Vec < K > > > , i : usize , j : usize , value : K) { if let Some (dist) = m_dist { dist [i] [j] = value ; } }
    };
}

set_object!()