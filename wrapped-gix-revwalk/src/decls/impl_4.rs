macro_rules! deps {
    () => {
        Parents!();
        Item!();
        Either!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Iterator for Parents < '_ , '_ > { type Item = Result < gix_hash :: ObjectId , iter_parents :: Error > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . backing { Either :: Left (it) => { for token in it { match token { Ok (gix_object :: commit :: ref_iter :: Token :: Tree { .. }) => continue , Ok (gix_object :: commit :: ref_iter :: Token :: Parent { id }) => return Some (Ok (id)) , Ok (_unused_token) => break , Err (err) => return Some (Err (err . into ())) , } } None } Either :: Right ((cache , it)) => it . next () . map (| r | r . map (| pos | cache . id_at (pos) . to_owned ()) . map_err (Into :: into)) , } } }
    };
}

impl_4!();