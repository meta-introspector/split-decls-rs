macro_rules! deps {
    () => {
        Deserialize!();
        Serialize!();
    };
}

macro_rules! CapturedCommand {
    () => {
        deps!();
        # [cfg_attr (feature = "serde_enabled" , derive (Serialize , Deserialize , Debug , Clone))] # [cfg_attr (not (feature = "serde_enabled") , derive (Debug , Clone))] pub struct CapturedCommand { pub program : String , pub args : Vec < String > , pub stdout : String , pub stderr : String , pub status : Option < i32 > , }
    };
}

CapturedCommand!()