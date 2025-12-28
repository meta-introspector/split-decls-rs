macro_rules! InstallConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct InstallConfig { pub prefix : Option < String > , pub sysconfdir : Option < String > , }
    };
}

InstallConfig!()