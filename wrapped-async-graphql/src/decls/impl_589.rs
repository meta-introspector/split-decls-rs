macro_rules! deps {
    () => {
        Data!();
        Response!();
        NextExecute!();
        ExtensionContext!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl NextExecute < '_ > { async fn internal_run (self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str > , data : Option < Data > ,) -> Response { let execute_data = match (self . execute_data , data) { (Some (mut data1) , Some (data2)) => { data1 . merge (data2) ; Some (data1) } (Some (data) , None) => Some (data) , (None , Some (data)) => Some (data) , (None , None) => None , } ; if let Some ((first , next)) = self . chain . split_first () { first . execute (ctx , operation_name , NextExecute { chain : next , execute_fut_factory : self . execute_fut_factory , execute_data , } ,) . await } else { (self . execute_fut_factory) (execute_data) . await } } # [doc = " Call the [Extension::execute] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str >) -> Response { self . internal_run (ctx , operation_name , None) . await } # [doc = " Call the [Extension::execute] function of next extension with context"] # [doc = " data."] pub async fn run_with_data (self , ctx : & ExtensionContext < '_ > , operation_name : Option < & str > , data : Data ,) -> Response { self . internal_run (ctx , operation_name , Some (data)) . await } }
    };
}

impl_589!()