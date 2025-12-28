macro_rules! deps {
    () => {
        Response!();
    };
}

macro_rules! hyper_response {
    () => {
        deps!();
        # [doc = " An HTTP response."] # [doc = ""] # [doc = " Obtain one of these by making a request with `hyper_clientconn_send`, then"] # [doc = " polling the executor unntil you get a `hyper_task` of type"] # [doc = " `HYPER_TASK_RESPONSE`. To figure out which request this response"] # [doc = " corresponds to, check the userdata of the task, which you should"] # [doc = " previously have set to an application-specific identifier for the"] # [doc = " request."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_response_status:            Get the HTTP-Status code of this response."] # [doc = " - hyper_response_version:           Get the HTTP version used by this response."] # [doc = " - hyper_response_reason_phrase:     Get a pointer to the reason-phrase of this response."] # [doc = " - hyper_response_reason_phrase_len: Get the length of the reason-phrase of this response."] # [doc = " - hyper_response_headers:           Gets a reference to the HTTP headers of this response."] # [doc = " - hyper_response_body:              Take ownership of the body of this response."] # [doc = " - hyper_response_free:              Free an HTTP response."] pub struct hyper_response (pub (super) Response < IncomingBody >) ;
    };
}

hyper_response!()