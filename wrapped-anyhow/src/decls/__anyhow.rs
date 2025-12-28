macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! __anyhow {
    () => {
        deps!();
        # [doc (hidden)] # [macro_export] macro_rules ! __anyhow { ($ msg : literal $ (,) ?) => ({ let error = $ crate :: __private :: format_err ($ crate :: __private :: format_args ! ($ msg)) ; error }) ; ($ err : expr $ (,) ?) => ({ use $ crate :: __private :: kind ::*; let error = match $ err { error => (& error) . anyhow_kind () . new (error) , } ; error }) ; ($ fmt : expr , $ ($ arg : tt) *) => { $ crate :: Error :: msg ($ crate :: __private :: format ! ($ fmt , $ ($ arg) *)) } ; }
    };
}

__anyhow!()