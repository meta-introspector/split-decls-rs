macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [doc = " One of three sideband types allowing to multiplex information over a single connection."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Channel { # [doc = " The usable data itself in any format."] Data = 1 , # [doc = " Progress information in a user-readable format."] Progress = 2 , # [doc = " Error information in a user readable format. Receiving it usually terminates the connection."] Error = 3 , }
    };
}

Channel!();