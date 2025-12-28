macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crossbeam_channel :: unbounded ; use lsp_types :: notification :: { Exit , Initialized , Notification } ; use lsp_types :: request :: { Initialize , Request } ; use lsp_types :: { InitializeParams , InitializedParams } ; use serde_json :: to_value ; use crate :: { Connection , Message , ProtocolError , RequestId } ; struct TestCase { test_messages : Vec < Message > , expected_resp : Result < (RequestId , serde_json :: Value) , ProtocolError > , } fn initialize_start_test (test_case : TestCase) { let (reader_sender , reader_receiver) = unbounded :: < Message > () ; let (writer_sender , writer_receiver) = unbounded :: < Message > () ; let conn = Connection { sender : writer_sender , receiver : reader_receiver } ; for msg in test_case . test_messages { assert ! (reader_sender . send (msg) . is_ok ()) ; } let resp = conn . initialize_start () ; assert_eq ! (test_case . expected_resp , resp) ; assert ! (writer_receiver . recv_timeout (std :: time :: Duration :: from_secs (1)) . is_err ()) ; } # [test] fn not_exit_notification () { let notification = crate :: Notification { method : Initialized :: METHOD . to_owned () , params : to_value (InitializedParams { }) . unwrap () , } ; let params_as_value = to_value (InitializeParams :: default ()) . unwrap () ; let req_id = RequestId :: from (234) ; let request = crate :: Request { id : req_id . clone () , method : Initialize :: METHOD . to_owned () , params : params_as_value . clone () , } ; initialize_start_test (TestCase { test_messages : vec ! [notification . into () , request . into ()] , expected_resp : Ok ((req_id , params_as_value)) , }) ; } # [test] fn exit_notification () { let notification = crate :: Notification { method : Exit :: METHOD . to_owned () , params : to_value (()) . unwrap () } ; let notification_msg = Message :: from (notification) ; initialize_start_test (TestCase { test_messages : vec ! [notification_msg . clone ()] , expected_resp : Err (ProtocolError :: new (format ! ("expected initialize request, got {notification_msg:?}"))) , }) ; } }
    };
}

tests!()