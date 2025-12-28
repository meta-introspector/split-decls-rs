macro_rules! Task {
    () => {
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct Task { pub name : String , pub description : String , pub status : String , # [serde (default)] pub depends_on : Vec < String > , # [serde (default)] pub command : Option < String > , # [serde (default)] pub path : Option < String > , }
    };
}

Task!()