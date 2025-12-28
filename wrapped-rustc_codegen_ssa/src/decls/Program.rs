macro_rules! Program {
    () => {
        # [derive (Clone)] enum Program { Normal (OsString) , CmdBatScript (OsString) , Lld (OsString , LldFlavor) , }
    };
}

Program!()