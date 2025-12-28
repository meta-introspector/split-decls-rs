macro_rules! deps {
    () => {
        Server!();
    };
}

macro_rules! Request {
    () => {
        deps!();
        # [doc = " A request to be handled by the server, typically done in a loop."] pub struct Request < 'a > { parent : & 'a mut Server , # [doc = " The command to execute with this request."] pub command : String , # [doc = " A list of key-value pairs of meta-data related to `command`."] pub meta : Vec < (String , BString) > , }
    };
}

Request!();