macro_rules! string_len {
    () => {
        # [doc = " Computes the total string length of a `fast_collect` result."] fn string_len < T : AsRef < str > > (vecs : & Either < Vec < T > , LinkedList < Vec < T > > >) -> usize { let strs = match vecs { Either :: Left (vec) => Either :: Left (vec . iter ()) , Either :: Right (list) => Either :: Right (list . iter () . flatten ()) , } ; strs . map (AsRef :: as_ref) . map (str :: len) . sum () }
    };
}

string_len!();