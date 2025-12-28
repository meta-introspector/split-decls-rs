macro_rules! deps {
    () => {
        StyledBuffer!();
        TitleStyle!();
        Element!();
        Title!();
        ElementStyle!();
        Renderer!();
        SourceMap!();
        Group!();
        LineAnnotation!();
        Origin!();
    };
}

macro_rules! render_short_message {
    () => {
        deps!();
        fn render_short_message (renderer : & Renderer , groups : & [Group < '_ >]) -> Result < String , fmt :: Error > { let mut buffer = StyledBuffer :: new () ; let mut labels = None ; let group = groups . first () . expect ("Expected at least one group") ; let Some (title) = & group . title else { panic ! ("Expected a Title") ; } ; if let Some (Element :: Cause (cause)) = group . elements . iter () . find (| e | matches ! (e , Element :: Cause (_))) { let labels_inner = cause . markers . iter () . filter_map (| ann | match & ann . label { Some (msg) if ann . kind . is_primary () => { if ! msg . trim () . is_empty () { Some (msg . to_string ()) } else { None } } _ => None , }) . collect :: < Vec < _ > > () . join (", ") ; if ! labels_inner . is_empty () { labels = Some (labels_inner) ; } if let Some (path) = & cause . path { let mut origin = Origin :: path (path . as_ref ()) ; let source_map = SourceMap :: new (& cause . source , cause . line_start) ; let (_depth , annotated_lines) = source_map . annotated_lines (cause . markers . clone () , cause . fold) ; if let Some (primary_line) = annotated_lines . iter () . find (| l | l . annotations . iter () . any (LineAnnotation :: is_primary)) . or (annotated_lines . iter () . find (| l | ! l . annotations . is_empty ())) { origin . line = Some (primary_line . line_index) ; if let Some (first_annotation) = primary_line . annotations . iter () . min_by_key (| a | (Reverse (a . is_primary ()) , a . start . char)) { origin . char_column = Some (first_annotation . start . char + 1) ; } } render_origin (renderer , & mut buffer , 0 , & origin , true , true , true , 0) ; buffer . append (0 , ": " , ElementStyle :: LineAndColumn) ; } } render_title (renderer , & mut buffer , title , 0 , TitleStyle :: MainHeader , false , 0 ,) ; if let Some (labels) = labels { buffer . append (0 , & format ! (": {labels}") , ElementStyle :: NoStyle) ; } let mut out_string = String :: new () ; buffer . render (& title . level , & renderer . stylesheet , & mut out_string) ? ; Ok (out_string) }
    };
}

render_short_message!()