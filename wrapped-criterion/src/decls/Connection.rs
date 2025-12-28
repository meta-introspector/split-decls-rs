macro_rules! deps {
    () => {
        InnerConnection!();
    };
}

macro_rules! Connection {
    () => {
        deps!();
        # [doc = " This is really just a holder to allow us to send messages through a shared reference to the"] # [doc = " connection."] # [derive (Debug)] pub struct Connection { inner : RefCell < InnerConnection > , }
    };
}

Connection!()