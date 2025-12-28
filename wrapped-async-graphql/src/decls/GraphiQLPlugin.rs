macro_rules! GraphiQLPlugin {
    () => {
        # [allow (missing_docs)] # [derive (Debug , Default , Serialize)] pub struct GraphiQLPlugin < 'a > { pub name : & 'a str , pub constructor : & 'a str , # [doc = " assets which would be placed in head"] pub head_assets : Option < & 'a str > , # [doc = " assets which would be placed in body"] pub body_assets : Option < & 'a str > , # [doc = " related configs which would be placed before loading plugin"] pub pre_configs : Option < & 'a str > , # [doc = " props which would be passed to the plugin's constructor"] pub props : Option < & 'a str > , }
    };
}

GraphiQLPlugin!()