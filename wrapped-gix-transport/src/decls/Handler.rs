macro_rules! deps {
    () => {
        StreamOrBuffer!();
    };
}

macro_rules! Handler {
    () => {
        deps!();
        # [derive (Default)] struct Handler { send_header : Option < pipe :: Writer > , send_data : Option < pipe :: Writer > , receive_body : Option < StreamOrBuffer > , checked_status : bool , last_status : usize , follow : FollowRedirects , }
    };
}

Handler!()