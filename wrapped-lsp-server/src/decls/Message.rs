macro_rules! deps {
    () => {
        Response!();
        Request!();
        Notification!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug , Clone)] # [serde (untagged)] pub enum Message { Request (Request) , Response (Response) , Notification (Notification) , }
    };
}

Message!()