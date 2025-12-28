macro_rules! deps {
    () => {
        Form!();
        Part!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Form { # [doc = " Creates a new blank form ready for the addition of new data."] pub fn new () -> Form { Form { head : ptr :: null_mut () , tail : ptr :: null_mut () , headers : Vec :: new () , buffers : Vec :: new () , strings : Vec :: new () , } } # [doc = " Prepares adding a new part to this `Form`"] # [doc = ""] # [doc = " Note that the part is not actually added to the form until the `add`"] # [doc = " method is called on `Part`, which may or may not fail."] pub fn part < 'a , 'data > (& 'a mut self , name : & 'data str) -> Part < 'a , 'data > { Part { error : None , form : self , name , array : vec ! [curl_sys :: curl_forms { option : curl_sys :: CURLFORM_END , value : ptr :: null_mut () , }] , } } }
    };
}

impl_38!()