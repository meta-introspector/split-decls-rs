macro_rules! split_vec {
    () => {
        # [doc = " Divide given vector in two equally sized vectors."] # [doc = " Return `None` if initial size is <=1."] # [doc = " We return the first half and keep the last half in `v`."] fn split_vec < T > (v : & mut Vec < T >) -> Option < Vec < T > > { if v . len () <= 1 { None } else { let n = v . len () / 2 ; Some (v . split_off (n)) } }
    };
}

split_vec!()