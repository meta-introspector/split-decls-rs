macro_rules! deps {
    () => {
        State!();
        Action!();
        VersionVec!();
        Channel!();
        Access!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl State { pub (super) fn check_for_leaks (& self , index : usize) { if self . msg_cnt != 0 { if self . created . is_captured () { panic ! ("Messages leaked.\n  \
                    Channel created: {}\n            \
                    Index: {}\n        \
                    Messages: {}" , self . created , index , self . msg_cnt) ; } else { panic ! ("Messages leaked.\n     Index: {}\n  Messages: {}" , index , self . msg_cnt) ; } } } pub (super) fn last_dependent_access (& self , action : Action) -> Option < & Access > { match action { Action :: MsgSend => self . last_send_access . as_ref () , Action :: MsgRecv => self . last_recv_access . as_ref () , } } pub (super) fn set_last_access (& mut self , action : Action , path_id : usize , version : & VersionVec) { match action { Action :: MsgSend => Access :: set_or_create (& mut self . last_send_access , path_id , version) , Action :: MsgRecv => Access :: set_or_create (& mut self . last_recv_access , path_id , version) , } } }
    };
}

impl_109!();