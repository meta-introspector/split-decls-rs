macro_rules! OsStr {
    () => {
        # [doc = " A UTF-8-encoded fixed string"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** To support dynamic values (i.e. `OsString`), enable the `string`"] # [doc = " feature"] # [doc = ""] # [doc = " </div>"] # [derive (Default , Clone , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct OsStr { name : Inner , }
    };
}

OsStr!()