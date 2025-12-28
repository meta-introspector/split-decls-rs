macro_rules! deps {
    () => {
        EncoderImpl!();
        Writer!();
        Config!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! encode_into_writer {
    () => {
        deps!();
        # [doc = " Encode the given value into a custom [Writer]."] # [doc = ""] # [doc = " See the [config] module for more information on configurations."] # [doc = ""] # [doc = " [config]: config/index.html"] pub fn encode_into_writer < E : enc :: Encode , W : Writer , C : Config > (val : E , writer : W , config : C ,) -> Result < () , error :: EncodeError > { let mut encoder = enc :: EncoderImpl :: < _ , C > :: new (writer , config) ; val . encode (& mut encoder) ? ; Ok (()) }
    };
}

encode_into_writer!();