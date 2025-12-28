macro_rules! Mutation {
    () => {
        # [doc = " A helper struct that collects the arguments for [`HttpServer::check_authorized`]."] # [doc = " Based on looking at the request, these are the fields that the authentication header should attest to."] struct Mutation < 'a > { mutation : & 'a str , name : Option < & 'a str > , vers : Option < & 'a str > , cksum : Option < & 'a str > , }
    };
}

Mutation!()