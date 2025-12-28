macro_rules! deps {
    () => {
        Change!();
        Case!();
        Input!();
        Color!();
        Error!();
        Specified!();
        Color256!();
        ColorRgb!();
        Result!();
        ErrorDetail!();
        Color16!();
    };
}

macro_rules! attr {
    () => {
        deps!();
        # [doc = " Parses any attributes inside a color tag."] fn attr (input : Input < '_ >) -> Result < '_ , Change > { let mut parser = alt ((style_attr , map (tuple ((color_kind_specifier , specified_color)) , | (kind , color) | kind . to_change (color)) , map (color_16 (Case :: Lowercase) , | color_16 | Change :: Foreground (Color :: Color16 (color_16))) , map (color_256 (Specified :: False) , | (color_256 , color_kind) | color_kind . unwrap () . to_change (Color :: Color256 (color_256))) , map (color_rgb (Specified :: False) , | (color_rgb , color_kind) | color_kind . unwrap () . to_change (Color :: ColorRgb (color_rgb))) , map (color_16 (Case :: Uppercase) , | color_16 | Change :: Background (Color :: Color16 (color_16))) ,)) ; parser (input) . map_err (| e | { match e { Err :: Error (_) => { let msg = match alphanumeric1 :: < & str , Error > (input) { Ok ((_ , attr)) => format ! ("Unknown color attribute: <{attr}>") , Err (_) => "Unable to parse this attribute" . to_string () , } ; Err :: Failure (Error :: new (input , ErrorKind :: Alpha , Some (ErrorDetail :: new (input , msg)))) } e => e } }) }
    };
}

attr!()