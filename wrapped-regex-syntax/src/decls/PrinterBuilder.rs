macro_rules! PrinterBuilder {
    () => {
        # [doc = " A builder for constructing a printer."] # [doc = ""] # [doc = " Note that since a printer doesn't have any configuration knobs, this type"] # [doc = " remains unexported."] # [derive (Clone , Debug)] struct PrinterBuilder { _priv : () , }
    };
}

PrinterBuilder!();