macro_rules! deps {
    () => {
        FormError!();
        Form!();
    };
}

macro_rules! Part {
    () => {
        deps!();
        # [doc = " One part in a multipart upload, added to a `Form`."] pub struct Part < 'form , 'data > { form : & 'form mut Form , name : & 'data str , array : Vec < curl_sys :: curl_forms > , error : Option < FormError > , }
    };
}

Part!()