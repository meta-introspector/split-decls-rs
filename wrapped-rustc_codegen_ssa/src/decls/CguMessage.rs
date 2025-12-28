macro_rules! CguMessage {
    () => {
        # [doc = " A message sent from the coordinator thread to the main thread telling it to"] # [doc = " process another codegen unit."] pub struct CguMessage ;
    };
}

CguMessage!()